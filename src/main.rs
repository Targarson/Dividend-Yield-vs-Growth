use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Scenario {
    initial_investment: f64,
    initial_dividend_yield_percent: f64,
    annual_dividend_growth_percent: f64,
    years: u32,
}

#[derive(Debug, Clone)]
struct YearProjection {
    year: u32,
    yield_on_cost_percent: f64,
    annual_dividend: f64,
    cumulative_dividends: f64,
}

fn main() {
    println!("=== Kalkulačka Dividend Yield vs Dividend Growth ===");
    println!("Spočítá vývoj dividendového příjmu v čase na základě zadaných hodnot.\n");

    loop {
        let scenario = read_scenario();
        let projections = project_dividends(&scenario);
        print_projection(&scenario, &projections);

        if !ask_yes_no("\nChceš spočítat další scénář s jiným yield/growth? (a/n): ") {
            println!("\nHotovo. Díky za použití kalkulačky!");
            break;
        }
        println!();
    }
}

fn read_scenario() -> Scenario {
    let initial_investment = read_f64("Počáteční investice (např. 100000): ", 0.01, f64::MAX);
    let initial_dividend_yield_percent =
        read_f64("Počáteční dividend yield v % (např. 3.5): ", 0.0, 100.0);
    let annual_dividend_growth_percent =
        read_f64("Roční dividend growth v % (např. 8): ", -100.0, 200.0);
    let years = read_u32("Počet let projekce (např. 20): ", 1, 100);

    Scenario {
        initial_investment,
        initial_dividend_yield_percent,
        annual_dividend_growth_percent,
        years,
    }
}

fn project_dividends(s: &Scenario) -> Vec<YearProjection> {
    let mut rows = Vec::with_capacity(s.years as usize);
    let mut annual_dividend = s.initial_investment * (s.initial_dividend_yield_percent / 100.0);
    let growth_multiplier = 1.0 + (s.annual_dividend_growth_percent / 100.0);
    let mut cumulative = 0.0;

    for year in 1..=s.years {
        if year > 1 {
            annual_dividend *= growth_multiplier;
        }
        cumulative += annual_dividend;

        rows.push(YearProjection {
            year,
            yield_on_cost_percent: (annual_dividend / s.initial_investment) * 100.0,
            annual_dividend,
            cumulative_dividends: cumulative,
        });
    }

    rows
}

fn print_projection(s: &Scenario, rows: &[YearProjection]) {
    println!("\n--- Výsledek projekce ---");
    println!("Investice: {:.2}", s.initial_investment);
    println!("Startovní yield: {:.2}%", s.initial_dividend_yield_percent);
    println!(
        "Roční growth dividendy: {:.2}%",
        s.annual_dividend_growth_percent
    );
    println!("Horizont: {} let\n", s.years);

    println!(
        "{:<5} | {:>14} | {:>18} | {:>22}",
        "Rok", "Yield on cost", "Roční dividenda", "Kumulativní dividendy"
    );
    println!("{}", "-".repeat(70));

    for r in rows {
        println!(
            "{:<5} | {:>13.2}% | {:>18.2} | {:>22.2}",
            r.year, r.yield_on_cost_percent, r.annual_dividend, r.cumulative_dividends
        );
    }

    if let Some(last) = rows.last() {
        let total_return_from_dividends =
            (last.cumulative_dividends / s.initial_investment) * 100.0;
        println!(
            "\nKonečný roční dividendový příjem: {:.2}",
            last.annual_dividend
        );
        println!(
            "Yield on cost v posledním roce: {:.2}%",
            last.yield_on_cost_percent
        );
        println!(
            "Kumulativní dividendy za celé období: {:.2} ({:.2}% původní investice)",
            last.cumulative_dividends, total_return_from_dividends
        );
    }
}

fn ask_yes_no(prompt: &str) -> bool {
    loop {
        print!("{}", prompt);
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Chyba při čtení vstupu, zkus to znovu.");
            continue;
        }

        match input.trim().to_lowercase().as_str() {
            "a" | "ano" | "y" | "yes" => return true,
            "n" | "ne" | "no" => return false,
            _ => println!("Prosím napiš 'a' nebo 'n'."),
        }
    }
}

fn read_f64(prompt: &str, min: f64, max: f64) -> f64 {
    loop {
        print!("{}", prompt);
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Chyba při čtení vstupu, zkus to znovu.");
            continue;
        }

        let normalized = input.trim().replace(',', ".");
        match normalized.parse::<f64>() {
            Ok(value) if value >= min && value <= max => return value,
            Ok(_) => println!("Hodnota musí být mezi {min} a {max}."),
            Err(_) => println!("Neplatné číslo, zkus to prosím znovu."),
        }
    }
}

fn read_u32(prompt: &str, min: u32, max: u32) -> u32 {
    loop {
        print!("{}", prompt);
        let _ = io::stdout().flush();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Chyba při čtení vstupu, zkus to znovu.");
            continue;
        }

        match input.trim().parse::<u32>() {
            Ok(value) if value >= min && value <= max => return value,
            Ok(_) => println!("Hodnota musí být mezi {min} a {max}."),
            Err(_) => println!("Neplatné celé číslo, zkus to prosím znovu."),
        }
    }
}
