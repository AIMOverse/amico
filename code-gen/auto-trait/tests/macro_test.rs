use amico_auto_trait::{make_dynamic, make_local};
use std::future::Future;
use std::time::Duration;

#[make_dynamic]
#[make_local]
trait AsyncTrait {
    fn process_num(&self, num: i32) -> impl Future<Output = i32> + Send;
}

struct Test;

impl AsyncTrait for Test {
    async fn process_num(&self, num: i32) -> i32 {
        tokio::time::sleep(Duration::from_millis(num as u64 * 100)).await;
        num
    }
}

// Another implementation of AsyncTrait
struct AnotherTest;

impl AsyncTrait for AnotherTest {
    async fn process_num(&self, num: i32) -> i32 {
        // This implementation doubles the number without sleeping
        num * 2
    }
}

#[tokio::test]
async fn test_trait_gen() {
    println!("Testing the original trait, Dyn trait, and Local trait implementations...");

    // Test original AsyncTrait implementation
    let test = Test;
    let result = test.process_num(1).await;

    // Test AsyncTraitDyn with _dyn suffix methods
    let test_dyn: Box<dyn AsyncTraitDyn> = Box::new(Test);
    let result_dyn = test_dyn.process_num_dyn(1).await;

    assert_eq!(result, result_dyn);

    // Test AsyncTraitLocal with _local suffix methods
    let test_local: &dyn AsyncTraitLocal = &Test;
    let result_local = test_local.process_num_local(1).await;

    assert_eq!(result, result_local);
}

#[tokio::test]
async fn test_multiple_impls() {
    // Create two implementations of AsyncTrait
    let test1 = Test;
    let test2 = AnotherTest;

    // Test original trait
    let results = vec![test1.process_num(1).await, test2.process_num(2).await];

    // Test through dynamic dispatch
    let dyn_impls: Vec<Box<dyn AsyncTraitDyn>> = vec![Box::new(test1), Box::new(AnotherTest)];

    for (i, implementation) in dyn_impls.iter().enumerate() {
        let result = implementation.process_num_dyn(i as i32 + 1).await;
        assert_eq!(result, results[i]);
    }
}
