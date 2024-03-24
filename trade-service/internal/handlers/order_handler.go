package handlers

import (
	"encoding/json"
	"net/http"
	"trade-service/internal/models"
	"trade-service/internal/services"
)

// OrderHandler holds the services needed for handling order-related requests.
type OrderHandler struct {
	orderService *services.OrderService
}

// NewOrderHandler creates a new instance of OrderHandler.
func NewOrderHandler(orderService *services.OrderService) *OrderHandler {
	return &OrderHandler{
		orderService: orderService,
	}
}

// CreateOrder handles the HTTP request for creating a new order.
func (h *OrderHandler) CreateOrder(w http.ResponseWriter, r *http.Request) {
	var order models.Order

	// Decode the incoming JSON payload into an order struct.
	if err := json.NewDecoder(r.Body).Decode(&order); err != nil {
		http.Error(w, "Error decoding request body", http.StatusBadRequest)
		return
	}

	// Call the order service to create a new order in the database.
	result, err := h.orderService.CreateOrder(r.Context(), order)
	if err != nil {
		http.Error(w, "Error creating order", http.StatusInternalServerError)
		return
	}

	// Respond to the client.
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(result)
}
