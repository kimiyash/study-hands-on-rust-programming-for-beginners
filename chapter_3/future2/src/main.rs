use futures::executor;

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn somthing_great_async_function() -> i32 {
    let ans1 = async_add(2, 3).await;
    let ans2 = async_add(3, 4).await;
    let ans3 = async_add(4, 5).await;
    let result = ans1 + ans2 + ans3;
    println!("{}", result);
    result
}

async fn print_result(value: i32) {
    println!("{}", value);
}

async fn calculate() -> i32 {
    let add1 = async_add(2, 3).await;
    print_result(add1).await;
    let add2 = async_add(3, 4).await;
    print_result(add2).await;
    async_add(add1, add2).await
}

fn main() {
    executor::block_on(somthing_great_async_function());
    executor::block_on(calculate());
}
