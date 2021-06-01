use std::env;

use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

use types::Entry;

use crate::types::AnalyticsMessageData;

mod load;
mod types;
mod analytics;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Loading entries from {}...", filename);

    let entries = match load::load_entries_from_file(filename) {
        Ok(entries) => entries,
        Err(_) => panic!("Input parsing error")
    };

    println!("Calculating some analytics...");
    calculate_analytics_async(Arc::new(entries));
}


fn calculate_analytics_async(entries: Arc<Vec<Entry>>){
    let (sender, receiver) = mpsc::channel();

    let display_handle = spawn_display_thread(receiver);

    let hvf_handle = spawn_hvf_thread(entries.clone(), sender.clone());
    let alterations_handle = spawn_alterations_thread(entries.clone(), sender.clone());

    let helmsthread = thread::spawn(move || {
        println!("[Helmsthread] I'm the helmsman thread!");
        hvf_handle.join().unwrap();
        alterations_handle.join().unwrap();

        sender.send(AnalyticsMessageData::Stop()).unwrap();

        println!("[Helmsthread] Just sent a 'stop' message to the display");

        display_handle.join().unwrap();

        println!("[Helmsthread] All of the threads have terminated, I'm gonna have a rest as well.");
    });

    helmsthread.join().unwrap();
}

fn spawn_hvf_thread(entries: Arc<Vec<Entry>>, sender: Sender<AnalyticsMessageData>) -> JoinHandle<()> {
    thread::spawn(move || {
        let hvf = analytics::calculate_hidden_values_frequencies(&entries); 
        sender.send(AnalyticsMessageData::HiddenValuesFrequencies(hvf))
              .expect("[HVF] Unable to send hidden values frequencies");
        println!("[HVF] Calculated hidden values frequencies.");
    })
}

fn spawn_alterations_thread(entries: Arc<Vec<Entry>>, sender: Sender<AnalyticsMessageData>) -> JoinHandle<()> {
    thread::spawn(move || {
        let alterations= analytics::calculate_alterations(&entries); 
        sender.send(AnalyticsMessageData::Alterations(alterations))
              .expect("[Alterations] Unable to send alterations");
        println!("[Alterations] Calculated alterations.");
    })
}

fn spawn_display_thread(receiver: Receiver<AnalyticsMessageData>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            match receiver.recv().unwrap() {
                AnalyticsMessageData::HiddenValuesFrequencies(v) => 
                    println!("[Display] Hidden values frequencies: {:?}", v),
                AnalyticsMessageData::Alterations(v) => 
                    println!("[Display] Alterations: {:?}", v),
                AnalyticsMessageData::Stop() => {
                    println!("[Display] Received a 'stop' message, breaking the loop");
                    break;
                }
            }
        }
    })
}

#[allow(dead_code)]
fn calculate_analytics_sync(entries: &Vec<Entry>) {
    let hidden_values_frequencies = analytics::calculate_hidden_values_frequencies(entries); 
    let alterations = analytics::calculate_alterations(entries);

    println!("Hidden values frequencies: {:?}", hidden_values_frequencies);
    println!("Alterations: {}/{}", alterations, entries.len());
}