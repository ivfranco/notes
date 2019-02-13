use lexers::html::lex_html;
use lexers::simple_c::lex_simple_c;

fn main() {
    exercise_3_1_1();
    exercies_3_1_2();
}

fn exercise_3_1_1() {
    let input = "float limitedSquare(x) float x; {
/* returns x-squared, but never more than 100 */
return (x<=-10.0||x>=10.0)?100:x*x;
}";

    for token in lex_simple_c(input) {
        println!("{:?}", token);
    }
    println!("");
}

fn exercies_3_1_2() {
    let input = r#"Here is a photo of <B>my house</B>:
<P><IMG SRC = "house.gif"><BR>
See <A HREF = "morePix.html">More Pictures</A> if you
liked that one.<P>"#;

    for token in lex_html(input) {
        println!("{:?}", token);
    }
    println!("");
}
