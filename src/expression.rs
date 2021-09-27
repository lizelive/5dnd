use rand::prelude::*;

type Number = f64;
type Integer = i64;

struct Dice{
    sides:Integer,
    count: Integer,
    modifer:Integer,
    exploding: bool,
}

impl Dice {
    fn new(sides: Integer, count: Integer) -> Self { Self { sides, count, modifer: 0, exploding: false } }

    fn roll(&self) -> Integer {
        let mut rng = thread_rng();
        let mut acc = self.modifer;
        for i in 0..self.count{
            let mut val = rng.gen_range(1..=self.sides);
            while val==self.sides && self.exploding {
                acc+= val;
                val = rng.gen_range(1..=self.sides);
            }
            acc += val;
        }
        acc
    }
}

enum BinaryOperator {
    Eq,
    Gt,
    Gte,
    Lt,
    Lte,
    Neq,

    And,
    Or,
    Xor,

    Mod,

    Add,
    Sub,
    Mul,
    Div,

    Max,
    Min,
}

enum Stat {
    Zero,
    Named(String),
    Derived(Box<Stat>, Integer),
}

enum MonoOperator {
    Not,
}

use std::cmp::Ordering;

#[derive(Eq)]
enum Value {
    Integer(Integer),
    Boolean(bool)
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        let l: Integer = (*self).into();
        let r: Integer = (*other).into();
        l.cmp(&r) 
    }
}

impl PartialOrd for Value  {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



impl From<Value> for Integer {
    fn from(val: Value) -> Self {
        match val {
            Value::Integer(v) => v,
            Value::Boolean(true) => 1,
            Value::Boolean(false) => 0,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other){
            (Value::Integer(l), Value::Integer(r)) => l.eq(r),
            (Value::Boolean(l), Value::Boolean(r)) => l.eq(r),

            (Value::Integer(_), Value::Boolean(_)) => other.eq(self),

            (Value::Boolean(false), Value::Integer(0)) => true,
            (Value::Boolean(true), Value::Integer(0)) => false,
            (Value::Boolean(true), Value::Integer(_)) => true,
            (Value::Boolean(false), Value::Integer(_)) => false,

        }
    }
}
impl From<Value> for bool {
    fn from(val: Value) -> Self {
        match val{
            Value::Integer(0) => false,
            Value::Integer(_) => true,
            Value::Boolean(v) => v,
        }
    }
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::Boolean(v)
    }
}

impl From<Integer> for Value {
    fn from(v: Integer) -> Self {
        Self::Integer(v)
    }
}

enum  Modifier{
    // negative values gives dissadvantage
    Tries(Stat, Integer),
    Modifier(Stat, Dice),
}

enum Expression {
    Tries { expr: Box<Expression>, tries: Integer },
    Integer { value: Integer },
    Dice { dice: Dice },
    Roll { stat: Box<Stat> },
    BinaryOperator { operator: BinaryOperator, left: Box<Expression>, right: Box<Expression> },
    If { cond: Box<Expression>, t: Box<Expression>, f: Box<Expression> },
}
use std::iter;

impl Expression {
    fn eval(&self) -> Value{
        match self {
            Expression::Integer { value } => (*value).into(),
            Expression::Dice { dice } => dice.roll().into(),
            Expression::Tries { expr, tries } => {
                let values = iter::repeat(expr).take(tries.abs() as usize).map( | expr | expr.eval());
                if *tries > 0{
                    values.max()
                }
                else {
                    values.min()
                }.expect("cannot have zero tries")
            },
            Expression::Roll { stat } => todo!(),
            Expression::BinaryOperator { operator, left, right } => {
                let l = left.eval();
                let r = right.eval();

                match operator{
                    BinaryOperator::Eq => Value::Boolean(l == r),
                    BinaryOperator::Gt => Value::Boolean(l > r),
                    BinaryOperator::Gte => Value::Boolean(l >= r),
                    BinaryOperator::Lt => Value::Boolean(l < r),
                    BinaryOperator::Lte => Value::Boolean(l <= r),
                    BinaryOperator::Neq => Value::Boolean(l != r),
                    BinaryOperator::And => Value::Boolean(bool::from(l) && bool::from(r)),
                    BinaryOperator::Or => Value::Boolean(bool::from(l) || bool::from(r)),
                    BinaryOperator::Xor => Value::Boolean(bool::from(l) ^ bool::from(r)),
                    BinaryOperator::Mod => todo!(),
                    BinaryOperator::Add => todo!(),
                    BinaryOperator::Sub => todo!(),
                    BinaryOperator::Mul => todo!(),
                    BinaryOperator::Div => todo!(),
                    BinaryOperator::Max => todo!(),
                    BinaryOperator::Min => todo!(),
                }
            },
            Expression::If { cond, t, f } => {
                if cond.eval().into(){
                    t.eval()
                }
                else {
                    f.eval()
                }
            },
        }
    }
}