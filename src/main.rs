use cursive::views::TextView;

fn main() {
    let mut siv = cursive::default();

    siv.add_layer(TextView::new("Hello World"));

    siv.run();
}
