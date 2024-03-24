package main

import (
	"log"
	"net/http"
	"os"

	"trade-service/internal/db"
	"trade-service/internal/handlers"
	"trade-service/internal/services"
)

func main() {
	mongodbURI := os.Getenv("MONGODB_URI")
	if mongodbURI == "" {
		log.Fatal("MONGODB_URI is not set in .env file")
	}

	// Establish a connection to MongoDB
	client := db.Connect(mongodbURI)

	// Initialize services and handlers
	tradingPairService := services.NewTradingPairService(client)
	tradingPairHandler := handlers.NewTradingPairHandler(tradingPairService)
	orderService := services.NewOrderService(client)
	orderHandler := handlers.NewOrderHandler(orderService)

	// Define HTTP routes
	http.HandleFunc("/market", func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodPost {
			tradingPairHandler.CreateTradingPair(w, r)
		} else {
			// Respond with an error if the request method is not POST
			http.Error(w, "Invalid request method", http.StatusMethodNotAllowed)
		}
	})

	http.HandleFunc("/order", func(w http.ResponseWriter, r *http.Request) {
		switch r.Method {
		case http.MethodPost:
			orderHandler.CreateOrder(w, r)
		default:
			http.Error(w, "Invalid request method", http.StatusMethodNotAllowed)
		}
	})

	// Start listening for HTTP requests
	log.Println("Server starting on :8080")
	if err := http.ListenAndServe(":8080", nil); err != nil {
		log.Fatal("ListenAndServe:", err)
	}
}
