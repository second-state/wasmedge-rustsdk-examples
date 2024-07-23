use std::env;
use wasmedge_wasi_nn as wasi_nn;

fn main() {
    let args: Vec<String> = env::args().collect();
    let model_name: &str = &args[1];

    let result =
        wasi_nn::GraphBuilder::new(wasi_nn::GraphEncoding::Ggml, wasi_nn::ExecutionTarget::CPU)
            .build_from_cache(model_name);
    let graph = match result {
        Ok(graph) => graph,
        Err(err) => {
            println!("Failed to build graph: {:?}", err);
            return;
        }
    };

    let mut context = graph.init_execution_context().unwrap();

    let system_prompt = String::from("<<SYS>>You are a helpful, respectful and honest assistant. Always answer as short as possible, while being safe. <</SYS>>");
    let mut chat_history = String::new();

    loop {
        println!("[Question]:");
        let input = read_input();

        let prompt = match chat_history.is_empty() {
            true => format!(
                "<s>[INST] <<SYS>>\n{system_prompt}\n<</SYS>>\n\n{user_message} [/INST]",
                system_prompt = system_prompt.trim(),
                user_message = input.trim()
            ),
            false => format!(
                "{chat_history}<s>[INST] {user_message} [/INST]",
                chat_history = chat_history.trim(),
                user_message = input.trim()
            ),
        };

        let tensor_data = prompt.trim().as_bytes().to_vec();
        context
            .set_input(0, wasi_nn::TensorType::U8, &[1], &tensor_data)
            .unwrap();

        // Execute the inference.
        context.compute().unwrap();

        // Retrieve the output.
        let mut output_buffer = vec![0u8; 2048];
        let output_size = context.get_output(0, &mut output_buffer).unwrap();
        let output = String::from_utf8_lossy(&output_buffer[..output_size]).to_string();
        println!("[Answer]:\n{}", output.trim());

        chat_history = format!(
            "{chat_history} {model_answer} </s>",
            chat_history = prompt.trim(),
            model_answer = output.trim(),
        );

        // println!("=== saved_prompt ====");
        // println!("{}", chat_history);
    }
}

fn read_input() -> String {
    loop {
        let mut answer = String::new();
        std::io::stdin()
            .read_line(&mut answer)
            .ok()
            .expect("Failed to read line");
        if !answer.is_empty() && answer != "\n" && answer != "\r\n" {
            return answer;
        }
    }
}
