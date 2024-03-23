package models

import (
	"fmt"

	"go.mongodb.org/mongo-driver/bson/primitive"
)

// TradingPair represents a pair of teams for event trading.
type TradingPair struct {
    ID    primitive.ObjectID `bson:"_id,omitempty"` // MongoDB ID
    Base  string             `bson:"base"`
    Quote string             `bson:"quote"`
}

// NewTradingPair creates a new TradingPair instance.
func NewTradingPair(base, quote string) *TradingPair {
    return &TradingPair{Base: base, Quote: quote}
}

// ToString returns the trading pair formatted as "Base_Quote".
func (tp *TradingPair) ToString() string {
    return fmt.Sprintf("%s_%s", tp.Base, tp.Quote)
}
