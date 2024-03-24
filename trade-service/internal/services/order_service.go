package services

import (
	"context"
	"time"
	"trade-service/internal/models"

	"go.mongodb.org/mongo-driver/mongo"
)

type OrderService struct {
	db *mongo.Collection
}

func NewOrderService(client *mongo.Client) *OrderService {
	return &OrderService{
		db: client.Database("trade").Collection("orders"),
	}
}

// CreateOrder inserts a new order into the database.
func (s *OrderService) CreateOrder(ctx context.Context, order models.Order) (*mongo.InsertOneResult, error) {
	// Set the CreatedAt field to the current time
	order.CreatedAt = time.Now()

	// Insert the order into the database
	result, err := s.db.InsertOne(ctx, order)
	if err != nil {
		return nil, err
	}

	return result, nil
}
