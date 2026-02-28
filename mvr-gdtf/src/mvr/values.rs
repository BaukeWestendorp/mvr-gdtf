pub type FileName = String;

use regex::Regex;
use std::{
    fmt,
    str::{self, FromStr as _},
};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct CieColor {
    pub x: f32,
    pub y: f32,
    pub yy: f32,
}

impl str::FromStr for CieColor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(format!("CieColor must have 3 comma-separated values: {}", s));
        }
        let x = parts[0].trim().parse::<f32>().map_err(|e| format!("Failed to parse x: {}", e))?;
        let y = parts[1].trim().parse::<f32>().map_err(|e| format!("Failed to parse y: {}", e))?;
        let yy =
            parts[2].trim().parse::<f32>().map_err(|e| format!("Failed to parse yy: {}", e))?;
        Ok(CieColor { x, y, yy })
    }
}

impl fmt::Display for CieColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.yy)
    }
}

impl TryFrom<String> for CieColor {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        CieColor::from_str(&value)
    }
}

impl From<CieColor> for String {
    fn from(c: CieColor) -> Self {
        c.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Matrix {
    pub u1: f32,
    pub u2: f32,
    pub u3: f32,
    pub v1: f32,
    pub v2: f32,
    pub v3: f32,
    pub w1: f32,
    pub w2: f32,
    pub w3: f32,
    pub o1: f32,
    pub o2: f32,
    pub o3: f32,
}

impl str::FromStr for Matrix {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\{([^\}]*)\}").unwrap();
        let mut vals = Vec::with_capacity(12);
        for cap in re.captures_iter(s) {
            let group = &cap[1];
            for num in group.split(',') {
                let val = num
                    .trim()
                    .parse::<f32>()
                    .map_err(|e| format!("Failed to parse matrix value: {}", e))?;
                vals.push(val);
            }
        }
        if vals.len() != 12 {
            return Err(format!("Matrix must have 12 values in 4 groups of 3: {}", s));
        }
        Ok(Matrix {
            u1: vals[0],
            u2: vals[1],
            u3: vals[2],
            v1: vals[3],
            v2: vals[4],
            v3: vals[5],
            w1: vals[6],
            w2: vals[7],
            w3: vals[8],
            o1: vals[9],
            o2: vals[10],
            o3: vals[11],
        })
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{{},{},{}}}{{{},{},{}}}{{{},{},{}}}{{{},{},{}}}",
            self.u1,
            self.u2,
            self.u3,
            self.v1,
            self.v2,
            self.v3,
            self.w1,
            self.w2,
            self.w3,
            self.o1,
            self.o2,
            self.o3
        )
    }
}

impl TryFrom<String> for Matrix {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Matrix::from_str(&value)
    }
}

impl From<Matrix> for String {
    fn from(m: Matrix) -> Self {
        m.to_string()
    }
}
