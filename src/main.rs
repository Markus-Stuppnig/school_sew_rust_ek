/// The main function that runs the three parts of the program.
fn main() {
    teil_1();
    teil_2();
    teil_3();
}

/// Part 1: Demonstrates vector operations and summing elements.
fn teil_1() {
    println!("Running: Teil 1");

    // Create an empty vector of i32
    let mut vector: Vec<i32> = vec![];

    // Push elements into the vector
    vector.push(4);
    vector.push(7);
    vector.push(9);

    // Initialize sum variable
    let mut sum: i32 = 0;

    println!("Elements in Vector:");

    // Iterate over the vector and sum the elements
    for elem in vector {
        println!("{}", elem);
        sum += elem;
    }

    println!("Sum: {}", sum);
}

/// Part 2: Demonstrates operations on a custom `Point` struct.
fn teil_2() {
    println!("Running: Teil 2");

    // Create four points
    let mut p1: Point = Point::new(0.0, 0.0);
    let mut p2: Point = Point::new(0.0, 10.0);
    let p3: Point = Point::new(10.0, 10.0);
    let mut p4: Point = Point::new(10.0, 0.0);

    // Calculate and print the circumference of the quadrilateral
    println!("Circumference: {}", circumference(&p1, &p2, &p3, &p4));
    println!("P2 Position: {:?}", p2.get());

    // Translate points
    p1.translate(5.0, 5.0);
    p2.translate(5.0, 0.0);
    p4.set(10.0, 5.0);

    // Recalculate and print the circumference
    println!("Circumference: {}", circumference(&p1, &p2, &p3, &p4));

    // Print the absolute distance of p3 to the origin
    println!("P3 Absolute Distance to Origin: {}", p3.abs());
}

/// Calculates the circumference of a quadrilateral formed by four points.
///
/// # Arguments
///
/// * `p1` - A reference to the first point.
/// * `p2` - A reference to the second point.
/// * `p3` - A reference to the third point.
/// * `p4` - A reference to the fourth point.
///
/// # Returns
///
/// The circumference of the quadrilateral.
fn circumference(p1: &Point, p2: &Point, p3: &Point, p4: &Point) -> f64 {
    p1.distance(&p2) + p2.distance(&p3) + p3.distance(&p4) + p4.distance(&p1)
}

/// Part 3: Demonstrates a simple Rock-Paper-Scissors game.
fn teil_3() {
    println!("Running: Teil 3");

    // Create instances of RockPaperScissors
    let r1: RockPaperScissors = RockPaperScissors::Rock;
    let r2: RockPaperScissors = RockPaperScissors::Paper;
    let r3: RockPaperScissors = RockPaperScissors::Scissors;

    // Play the game and get the result
    let mut result1: Result = r1.play(&r2);

    println!("Result1: {:?}", result1);
    println!("Swapped Result1: {:?}", result1.swap());

    // Play the game with the same choices
    let result2: Result = r3.play(&r3);

    println!("Result2: {:?}", result2);
}

/// A struct representing a point in 2D space.
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    /// Creates a new `Point`.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the point.
    /// * `y` - The y-coordinate of the point.
    ///
    /// # Returns
    ///
    /// A new `Point` instance.
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Gets the coordinates of the point.
    ///
    /// # Returns
    ///
    /// A tuple containing the x and y coordinates.
    fn get(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    /// Sets the coordinates of the point.
    ///
    /// # Arguments
    ///
    /// * `x` - The new x-coordinate.
    /// * `y` - The new y-coordinate.
    fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }

    /// Translates the point by given offsets.
    ///
    /// # Arguments
    ///
    /// * `x_offset` - The offset to add to the x-coordinate.
    /// * `y_offset` - The offset to add to the y-coordinate.
    fn translate(&mut self, x_offset: f64, y_offset: f64) {
        self.set(self.x + x_offset, self.y + y_offset);
    }

    /// Calculates the distance to another point.
    ///
    /// # Arguments
    ///
    /// * `p` - A reference to another point.
    ///
    /// # Returns
    ///
    /// The distance between the two points.
    fn distance(&self, p: &Point) -> f64 {
        let x_distance = self.x - p.x;
        let y_distance = self.y - p.y;
        (x_distance * x_distance + y_distance * y_distance).sqrt()
    }

    /// Calculates the distance to the origin (0, 0).
    ///
    /// # Returns
    ///
    /// The distance to the origin.
    fn abs(&self) -> f64 {
        self.distance(&Point::new(0.0, 0.0))
    }
}

/// An enum representing the choices in Rock-Paper-Scissors.
#[derive(Debug)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

impl RockPaperScissors {
    /// Plays a game of Rock-Paper-Scissors against another choice.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to the other choice.
    ///
    /// # Returns
    ///
    /// The result of the game.
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

/// An enum representing the result of a Rock-Paper-Scissors game.
#[derive(Debug)]
enum Result {
    Win,
    Lose,
    Draw,
}

impl Result {
    /// Swaps the result (Win becomes Lose, Lose becomes Win, Draw remains Draw).
    ///
    /// # Returns
    ///
    /// The swapped result.
    fn swap(&mut self) -> Result {
        match self {
            Result::Win => Result::Lose,
            Result::Lose => Result::Win,
            Result::Draw => Result::Draw,
        }
    }
}
