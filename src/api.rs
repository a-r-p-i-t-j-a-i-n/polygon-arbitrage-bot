use actix_web::{get, web, HttpResponse, Responder};
use crate::db::Database;
use log::{error, info};
use std::sync::Arc;

#[get("/api/opportunities")]
pub async fn get_opportunities(db: web::Data<Arc<Database>>) -> impl Responder {
    info!("API: Fetching opportunities from database...");
    
    match db.get_recent_opportunities(50).await {
        Ok(opportunities) => {
            info!("API: Found {} opportunities", opportunities.len());
            
            let json_data: Vec<serde_json::Value> = opportunities.into_iter().map(|opp| {
                serde_json::json!({
                    "id": opp.id,
                    "timestamp": opp.timestamp.to_rfc3339(),
                    "buy_exchange": opp.buy_exchange,
                    "sell_exchange": opp.sell_exchange,
                    "buy_price": opp.buy_price,
                    "sell_price": opp.sell_price,
                    "estimated_profit": opp.estimated_profit,
                    "profit_percentage": opp.profit_percentage,
                    "created_at": opp.timestamp.to_rfc3339()
                })
            }).collect();
            
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "data": json_data
            }))
        },
        Err(e) => {
            error!("API: Database error in get_opportunities: {}", e);
            println!("API: Database error in get_opportunities: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Database query failed: {}", e)
            }))
        }
    }
}

#[get("/api/stats")]
pub async fn get_stats(db: web::Data<Arc<Database>>) -> impl Responder {
    info!("API: Fetching stats from database...");
    
    match db.get_stats().await {
        Ok((total, avg, max)) => {
            info!("API: Stats - total={}, avg={:.2}, max={:.2}", total, avg, max);
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "data": {
                    "total_opportunities": total,
                    "average_profit": avg,
                    "best_profit": max,
                    "runtime": "Active"
                }
            }))
        },
        Err(e) => {
            error!("API: Database error in get_stats: {}", e);
            println!("API: Database error in get_stats: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "error": format!("Stats query failed: {}", e)
            }))
        }
    }
}

#[get("/api/status")]
pub async fn get_status() -> impl Responder {
    info!("API: Status endpoint called");
    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "data": {
            "running": true,
            "last_check": chrono::Utc::now().format("%H:%M:%S").to_string(),
            "status": "Monitoring Polygon DEXs"
        }
    }))
}
