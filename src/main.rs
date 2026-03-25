mod delete_file_test;
mod create_file_test;

fn main() {
    // println!("Hello, world!");

    delete_file_test::delete_file_test("tmp_result");
    create_file_test::create_file_test("tmp_result");
}
