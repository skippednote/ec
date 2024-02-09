## Assignment 1 -

1. Create a struct **`Task`** with fields **`id`** and **`payload`**. The **`id`** should be a unique identifier for each task, and **`payload`** can be a string representing the actual work to be done.
2. Implement a function **`create_task(id: u32, payload: &str) -> Task`** that creates a new task with the given **`id`** and **`payload`**.
3. Create a struct **`Worker`** with a field **`id`** and a method **`process_task(&self, task: Task) -> String`** that simulates processing the task. The method should return a string indicating that the task has been processed.
4. Implement a function **`create_worker(id: u32) -> Worker`** that creates a new worker with the given **`id`**.
5. Set up a message-passing channel between a main thread and worker threads. Use the **`std::sync::mpsc`** module for this purpose.
6. In the main thread, create a vector of tasks and distribute them among worker threads using the message-passing channel. Each worker should process the received task and send back a completion message.
7. Print the completion messages from the main thread to verify that tasks have been processed successfully.
8. Handle any necessary synchronization between threads.
9. Implement a timeout mechanism for tasks. If a task takes longer than a specified duration to process, consider it failed, and print an appropriate message.