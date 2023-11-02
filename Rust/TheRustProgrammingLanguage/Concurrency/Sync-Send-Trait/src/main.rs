fn main() {
    // two concurrency concepts in the language: std::market traits Sync and Send
    // Send: indiciates that ownership of values of the type implementing Send can be transferred between threads
    // Sync: indiciates that is is safe for the type implementing Sync to be referenced from multiple threads
        // T is Sync if &T is Send
        // Sync is most similar meaning of phrase thread safe
}
