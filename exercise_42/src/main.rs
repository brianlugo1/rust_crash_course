// Deriving functionality such as being able to print(Debug),
// Clone (own) the data, and Copy (borrow) the data!
#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Worker,
        work_hours: 50,
    };
    let _boss = Employee {
        position: Position::Manager,
        work_hours: 45,
    };
    let _supervisor = Employee {
        position: Position::Supervisor,
        work_hours: 35,
    };
    println!("{:?}", me.position);
    println!("{:?}", me.work_hours);
    println!("{:?}", _boss.position);
    println!("{:?}", _boss.work_hours);
    println!("{:?}", _supervisor.position);
    println!("{:?}", _supervisor.work_hours);
    print_employee(me);
    print_employee(_boss);
    print_employee(_supervisor);
}
