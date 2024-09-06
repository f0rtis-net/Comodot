; ModuleID = 'lol'
source_filename = "lol"
target triple = "arm64-apple-darwin24.0.0"

define i64 @main() {
fn_body:
  %test1 = call i64 @test1()
  ret i64 %test1
}

define i64 @test1() {
fn_body:
  ret i64 -2
}
