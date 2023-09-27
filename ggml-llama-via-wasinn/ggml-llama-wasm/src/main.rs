use std::env;
use wasi_nn;

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
    let mut saved_prompt = String::new();

    loop {
        println!("[Question]:");
        let input = read_input();
        if saved_prompt == "" {
            saved_prompt = format!("[INST] {} {} [/INST]", system_prompt, input.trim());
        } else {
            saved_prompt = format!("{} [INST] {} [/INST]", saved_prompt, input.trim());
        }

        let tensor_data = saved_prompt.as_bytes().to_vec();
        context
            .set_input(0, wasi_nn::TensorType::U8, &[1], &tensor_data)
            .unwrap();

        // Execute the inference.
        context.compute().unwrap();

        // Retrieve the output.
        let mut output_buffer = vec![0u8; 1000];
        let output_size = context.get_output(0, &mut output_buffer).unwrap();
        let output = String::from_utf8_lossy(&output_buffer[..output_size]).to_string();
        println!("[Answer]: {}", output);
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
