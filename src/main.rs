use std::io;

#[derive(Debug)]
struct Student {
    name: String,
    total_marks: f64,
    num_subjects: u32,
}

impl Student {
    fn new(name: String, total_marks: f64, num_subjects: u32) -> Self {
        Student {
            name,
            total_marks,
            num_subjects,
        }
    }

    // Custom function to calculate average
    fn calculate_average(&self) -> f64 {
        if self.num_subjects == 0 {
            return 0.0;
        }
        self.total_marks / self.num_subjects as f64
    }

    // Function to assign grade based on average
    fn assign_grade(&self) -> char {
        let average = self.calculate_average();
        match average {
            avg if avg >= 90.0 => 'A',
            avg if avg >= 75.0 => 'B',
            avg if avg >= 60.0 => 'C',
            _ => 'D',
        }
    }

    // Function to get grade description
    fn get_grade_description(&self) -> &str {
        match self.assign_grade() {
            'A' => "Excellent",
            'B' => "Good",
            'C' => "Satisfactory",
            'D' => "Needs Improvement",
            _ => "Invalid",
        }
    }
}

fn main() {
    println!("=== Student Grade Management System ===\n");

    // Get student information
    let student = get_student_info();
    
    // Generate and display report card
    print_report_card(&student);
    
    // Generate PDF report
    generate_pdf_report(&student);
}

fn get_student_info() -> Student {
    println!("Enter student details:");
    
    // Get student name
    print!("Student Name: ");
    let name = read_input().trim().to_string();
    
    // Get total marks
    print!("Total Marks: ");
    let total_marks: f64 = loop {
        match read_input().trim().parse() {
            Ok(marks) if marks >= 0.0 => break marks,
            _ => {
                println!("Please enter a valid positive number for total marks:");
            }
        }
    };
    
    // Get number of subjects
    print!("Number of Subjects: ");
    let num_subjects: u32 = loop {
        match read_input().trim().parse() {
            Ok(subjects) if subjects > 0 => break subjects,
            _ => {
                println!("Please enter a valid positive number for subjects:");
            }
        }
    };
    
    Student::new(name, total_marks, num_subjects)
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}

fn print_report_card(student: &Student) {
    let average = student.calculate_average();
    let grade = student.assign_grade();
    let description = student.get_grade_description();
    
    println!("\n{}", "=".repeat(50));
    println!("               REPORT CARD");
    println!("{}", "=".repeat(50));
    println!("Student Name      : {}", student.name);
    println!("Total Marks       : {:.2}", student.total_marks);
    println!("Number of Subjects: {}", student.num_subjects);
    println!("Average Score     : {:.2}", average);
    println!("Grade             : {} ({})", grade, description);
    println!("{}", "=".repeat(50));
    
    // Performance summary
    println!("\nPerformance Summary:");
    match grade {
        'A' => println!("🌟 Outstanding performance! Keep up the excellent work!"),
        'B' => println!("👍 Good work! You're doing well!"),
        'C' => println!("📚 Satisfactory performance. There's room for improvement!"),
        'D' => println!("📖 Focus on your studies. You can do better!"),
        _ => println!("Invalid grade calculation."),
    }
    println!();
}

fn generate_pdf_report(student: &Student) {
    // Create a formatted text report first
    let report_content = format!(
        "╔══════════════════════════════════════════════════════════════╗\n\
         ║                      STUDENT REPORT CARD                    ║\n\
         ╠══════════════════════════════════════════════════════════════╣\n\
         ║                                                              ║\n\
         ║  Student Name      : {:<42} ║\n\
         ║  Total Marks       : {:<42.2} ║\n\
         ║  Number of Subjects: {:<42} ║\n\
         ║  Average Score     : {:<42.2} ║\n\
         ║  Grade             : {} ({:<35}) ║\n\
         ║                                                              ║\n\
         ║  Generated on      : {:<42} ║\n\
         ║                                                              ║\n\
         ╠══════════════════════════════════════════════════════════════╣\n\
         ║  Performance Summary:                                        ║\n\
         ║  {:<58} ║\n\
         ╚══════════════════════════════════════════════════════════════╝\n",
        student.name,
        student.total_marks,
        student.num_subjects,
        student.calculate_average(),
        student.assign_grade(),
        student.get_grade_description(),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        match student.assign_grade() {
            'A' => "Outstanding performance! Keep up the excellent work!",
            'B' => "Good work! You're doing well!",
            'C' => "Satisfactory performance. Room for improvement!",
            'D' => "Focus on your studies. You can do better!",
            _ => "Invalid grade calculation.",
        }
    );
    
    // Save as text file (easily convertible to PDF later)
    use std::fs;
    let filename = format!("{}_report_card.txt", student.name.replace(" ", "_"));
    
    match fs::write(&filename, report_content) {
        Ok(_) => {
            println!("✅ Report card saved as: {}", filename);
            println!("💡 Tip: You can convert this to PDF using online converters or tools like pandoc");
        }
        Err(e) => println!("❌ Error saving report card: {}", e),
    }
    
    // Also try to generate HTML version for easy PDF conversion
    generate_html_report(student);
}

fn generate_html_report(student: &Student) {
    let html_content = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Student Report Card</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            max-width: 600px;
            margin: 50px auto;
            padding: 20px;
            background-color: #f5f5f5;
        }}
        .report-card {{
            background: white;
            padding: 30px;
            border-radius: 10px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
        }}
        .header {{
            text-align: center;
            color: #2c3e50;
            border-bottom: 3px solid #3498db;
            padding-bottom: 15px;
            margin-bottom: 25px;
        }}
        .info-row {{
            display: flex;
            justify-content: space-between;
            margin: 10px 0;
            padding: 8px 0;
            border-bottom: 1px solid #ecf0f1;
        }}
        .label {{
            font-weight: bold;
            color: #34495e;
        }}
        .value {{
            color: #2c3e50;
        }}
        .grade {{
            font-size: 24px;
            font-weight: bold;
            color: {};
        }}
        .performance {{
            margin-top: 20px;
            padding: 15px;
            background-color: #ecf0f1;
            border-radius: 5px;
            text-align: center;
        }}
        .footer {{
            text-align: center;
            margin-top: 20px;
            color: #7f8c8d;
            font-size: 12px;
        }}
    </style>
</head>
<body>
    <div class="report-card">
        <div class="header">
            <h1>STUDENT REPORT CARD</h1>
        </div>
        
        <div class="info-row">
            <span class="label">Student Name:</span>
            <span class="value">{}</span>
        </div>
        
        <div class="info-row">
            <span class="label">Total Marks:</span>
            <span class="value">{:.2}</span>
        </div>
        
        <div class="info-row">
            <span class="label">Number of Subjects:</span>
            <span class="value">{}</span>
        </div>
        
        <div class="info-row">
            <span class="label">Average Score:</span>
            <span class="value">{:.2}</span>
        </div>
        
        <div class="info-row">
            <span class="label">Grade:</span>
            <span class="value grade">{} ({})</span>
        </div>
        
        <div class="performance">
            <strong>Performance Summary:</strong><br>
            {}
        </div>
        
        <div class="footer">
            Generated on: {}
        </div>
    </div>
</body>
</html>"#,
        match student.assign_grade() {
            'A' => "#27ae60",
            'B' => "#f39c12", 
            'C' => "#e67e22",
            'D' => "#e74c3c",
            _ => "#95a5a6",
        },
        student.name,
        student.total_marks,
        student.num_subjects,
        student.calculate_average(),
        student.assign_grade(),
        student.get_grade_description(),
        match student.assign_grade() {
            'A' => "🌟 Outstanding performance! Keep up the excellent work!",
            'B' => "👍 Good work! You're doing well!",
            'C' => "📚 Satisfactory performance. There's room for improvement!",
            'D' => "📖 Focus on your studies. You can do better!",
            _ => "Invalid grade calculation.",
        },
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    
    let html_filename = format!("{}_report_card.html", student.name.replace(" ", "_"));
    
    match std::fs::write(&html_filename, html_content) {
        Ok(_) => {
            println!("✅ HTML report card saved as: {}", html_filename);
            println!("💡 Open the HTML file in your browser and use 'Print to PDF' for a PDF version");
            
            // Get the full path
            let current_dir = std::env::current_dir().unwrap();
            let full_path = current_dir.join(&html_filename);
            println!("📁 File location: {}", full_path.display());
            
            // Try to open with system browser
            println!("🌐 Attempting to open in browser...");
            
            #[cfg(target_os = "linux")]
            {
                let _ = std::process::Command::new("xdg-open")
                    .arg(&html_filename)
                    .spawn();
            }
            
            #[cfg(target_os = "windows")]
            {
                let _ = std::process::Command::new("cmd")
                    .args(&["/C", "start", &html_filename])
                    .spawn();
            }
            
            #[cfg(target_os = "macos")]
            {
                let _ = std::process::Command::new("open")
                    .arg(&html_filename)
                    .spawn();
            }
            
            println!("📝 If browser didn't open automatically, copy this path to your browser:");
            println!("   file://{}", full_path.display());
        }
        Err(e) => println!("❌ Error saving HTML report card: {}", e),
    }
}
