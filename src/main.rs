use std::sync::Arc;
use tokio::task;

// Example asynchronous function to concatenate two strings
async fn foo(s1: &str, s2: &str) -> String {
    // Simulate asynchronous operation with tokio::time::sleep
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    format!("{} {}", s1, s2)
}

#[tokio::main]
async fn main() {
    // Variables to pass to the spawned tasks

    let fnames = vec!["ashu", "prerna", "krishna"];
    let lnames = vec!["pednekar", "agarwal", "raj"];

    // Loop to spawn tasks
    for i in 0..fnames.len() {
        // Clone the Arcs for each task
        // Arc wrapping variables to make them 'static
        let fname = Arc::new(fnames[i]);
        let lname = Arc::new(lnames[i]);

        let var1 = Arc::clone(&fname);
        let var2 = Arc::clone(&lname);

        // Spawn the task
        task::spawn(async move {
            // Call foo function inside the task
            let result = foo(&*var1, &*var2).await;
            println!("{}", result);
        });
    }

    // Wait for all tasks to finish
    // If you don't wait here, main might exit before tasks complete
    // causing tasks to be dropped and not executed.
    // You might want to handle this differently depending on your use case.
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
}
