fn main() {
    teil_1();
    teil_2();
    teil_3();
}

fn teil_1() {
    println!("Running: Teil 1");

    let mut vector: Vec<i32> = vec![];

    vector.push(4);
    vector.push(7);
    vector.push(9);

    let mut sum: i32 = 0;

    println!("Elements in Vector:");

    for elem in vector {
        println!("{}", elem);
        sum += elem;
    }

    println!("Sum: {}", sum);
}

fn teil_2() {
    println!("Running: Teil 2");

    let mut p1: Point = Point::new(0.0, 0.0);
    let mut p2: Point = Point::new(0.0, 10.0);
    let p3: Point = Point::new(10.0, 10.0);
    let mut p4: Point = Point::new(10.0, 0.0);

    println!("Circumference: {}", circumference(&p1, &p2, &p3, &p4));
    println!("P2 Position: {:?}", p2.get());

    p1.translate(5.0, 5.0);
    p2.translate(5.0, 0.0);
    p4.set(10.0, 5.0);

    println!("Circumference: {}", circumference(&p1, &p2, &p3, &p4));

    println!("P3 Absolute Distance to Origin: {}", p3.abs());
}

fn circumference(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> f64 {
    p1.distance(&p2) + p2.distance(&p3) + p3.distance(&p4) + p4.distance(&p1)
}

fn teil_3() {
    println!("Running: Teil 3");

    let r1: RockPaperScissors = RockPaperScissors::Rock;
    let r2: RockPaperScissors = RockPaperScissors::Paper;
    let r3: RockPaperScissors = RockPaperScissors::Scissors;

    let mut result1: Result = r1.play(&r2);

    println!("Result1: {:?}", result1);
    println!("Swapped Result1: {:?}", result1.swap());

    let result2: Result = r3.play(&r3);

    println!("Result2: {:?}", result2);
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn get(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    fn translate(&mut self, x_offset: f64, y_offset: f64) {
        self.set(self.x + x_offset, self.y + y_offset);
    }

    fn distance(&self, p: &Point) -> f64 {
        let x_distance = self.x - p.x;
        let y_distance = self.y - p.y;
        (x_distance * x_distance + y_distance * y_distance).sqrt()
    }

    fn abs(&self) -> f64 {
        self.distance(&Point::new(0.0, 0.0))
    }
}

#[derive(Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    fn play(&self, other: &RockPaperScissors) -> Result {
        match (self, other) {
            (RockPaperScissors::Rock, RockPaperScissors::Rock) => Result::Draw,
            (RockPaperScissors::Rock, RockPaperScissors::Paper) => Result::Lose,
            (RockPaperScissors::Rock, RockPaperScissors::Scissors) => Result::Win,
            (RockPaperScissors::Paper, RockPaperScissors::Rock) => Result::Win,
            (RockPaperScissors::Paper, RockPaperScissors::Paper) => Result::Draw,
            (RockPaperScissors::Paper, RockPaperScissors::Scissors) => Result::Lose,
            (RockPaperScissors::Scissors, RockPaperScissors::Rock) => Result::Lose,
            (RockPaperScissors::Scissors, RockPaperScissors::Paper) => Result::Win,
            (RockPaperScissors::Scissors, RockPaperScissors::Scissors) => Result::Draw,
        }
    }
}

#[derive(Debug)]
enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    fn swap(&mut self) -> Result {
        match self {
            Result::Win => Result::Lose,
            Result::Lose => Result::Win,
            Result::Draw => Result::Draw,
        }
    }
}
