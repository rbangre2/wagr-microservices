// internal/handlers/tradingpair_handler.go
package handlers

import (
	"encoding/json"
	"net/http"
	"trade-service/internal/models"
	"trade-service/internal/services"
)

// TradingPairHandler is responsible for handling HTTP requests for trading pairs.
type TradingPairHandler struct {
    service *services.TradingPairService
}

// NewTradingPairHandler creates a new TradingPairHandler with the given service.
func NewTradingPairHandler(service *services.TradingPairService) *TradingPairHandler {
    return &TradingPairHandler{service: service}
}

// CreateTradingPair handles the HTTP request for creating a new trading pair.
func (h *TradingPairHandler) CreateTradingPair(w http.ResponseWriter, r *http.Request) {
    var tradingPair models.TradingPair
    if err := json.NewDecoder(r.Body).Decode(&tradingPair); err != nil {
        http.Error(w, err.Error(), http.StatusBadRequest)
        return
    }

    if err := h.service.CreateTradingPair(r.Context(), tradingPair); err != nil {
        http.Error(w, err.Error(), http.StatusInternalServerError)
        return
    }

    w.WriteHeader(http.StatusCreated)
    json.NewEncoder(w).Encode(tradingPair)
}
