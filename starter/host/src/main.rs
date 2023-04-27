use wasmtime::*;

fn main() {
    let engine = Engine::default();
    let module =
        Module::from_file(&engine, "target/wasm32-unknown-unknown/debug/wasm.wasm").unwrap();

    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).unwrap();

    let allocate = instance.get_func(&mut store, "allocate").unwrap();
    let allocate = allocate.typed::<i32, i32>(&store).unwrap();

    let deallocate = instance.get_func(&mut store, "deallocate").unwrap();
    let deallocate = deallocate.typed::<(i32, i32), ()>(&store).unwrap();

    let hello = instance.get_func(&mut store, "hello").unwrap();
    let hello = hello.typed::<i32, i32>(&store).unwrap();

    let name = "World".to_string();
    let input_ptr = allocate.call(&mut store, name.len() as i32 + 1).unwrap();

    let mem = instance.get_memory(&mut store, "memory").unwrap();

    mem.write(&mut store, input_ptr as usize, name.as_bytes())
        .unwrap();
    mem.write(&mut store, name.len(), &[0]).unwrap();

    let output_ptr = hello.call(&mut store, input_ptr).unwrap();

    let mem_data = mem.data(&store);
    let output = mem_data[output_ptr as usize..]
        .iter()
        .take_while(|b| **b != 0)
        .cloned()
        .collect::<Vec<u8>>();
    let output = String::from_utf8(output).unwrap();

    println!("{}", output);

    deallocate
        .call(&mut store, (input_ptr, name.len() as i32 + 1))
        .unwrap();
    deallocate
        .call(&mut store, (output_ptr, output.len() as i32 + 1))
        .unwrap();
}
