fn greet_world(){
    let southern_germany = "Grüß Gott!";
    let chinese = "世界, 你好";
    let english = "world, hello";
    let regions = [southern_germany, chinese, english];
    for regions in regions.iter(){
        println!("{}", &regions);
    }
}

fn main() {
    greet_world();
}
