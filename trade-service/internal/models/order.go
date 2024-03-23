package models

import (
	"time"

	"go.mongodb.org/mongo-driver/bson/primitive"
)

type BidOrAsk string

const (
    Bid BidOrAsk = "Bid"
    Ask BidOrAsk = "Ask"
)

type OrderType string
const (
	Market OrderType = "Market"
	Limit OrderType = "Limit"
)
// Order represents a trading order.
type Order struct {
    ID        *primitive.ObjectID `bson:"_id,omitempty"`
    UID       *string             `bson:"uid,omitempty"`
    Size      float64             `bson:"size"`
    BidOrAsk  BidOrAsk            `bson:"bid_or_ask"`
    OrderType OrderType           `bson:"order_type"`
    CreatedAt *time.Time          `bson:"created_at,omitempty"`
}