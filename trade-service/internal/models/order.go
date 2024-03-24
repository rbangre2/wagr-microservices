package models

import (
	"time"

	"go.mongodb.org/mongo-driver/bson/primitive"
)

type OrderType string
type BidOrAsk string

const (
	Market OrderType = "market"
	Limit  OrderType = "limit"
	Bid    BidOrAsk  = "bid"
	Ask    BidOrAsk  = "ask"
)

type Order struct {
	ID            primitive.ObjectID `bson:"_id,omitempty"`
	TradingPairID primitive.ObjectID `bson:"tradingPairId,omitempty"`
	UserID           string             `bson:"uid"`
	Size          float64            `bson:"size"`
	Price         *float64           `bson:"price,omitempty"` // Nil for market orders
	Type          OrderType          `bson:"type"`
	Side          BidOrAsk           `bson:"side"`
	CreatedAt     time.Time          `bson:"created_at"`
}
