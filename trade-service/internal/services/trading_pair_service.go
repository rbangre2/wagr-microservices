package services

import (
	"context"
	"trade-service/internal/models"

	"go.mongodb.org/mongo-driver/mongo"
)

type TradingPairService struct {
    client *mongo.Client
}

func NewTradingPairService(client *mongo.Client) *TradingPairService {
    return &TradingPairService{client: client}
}

func (s *TradingPairService) CreateTradingPair(ctx context.Context, tradingPair models.TradingPair) error {
    collection := s.client.Database("trade").Collection("markets")
    _, err := collection.InsertOne(ctx, tradingPair)
    if err != nil {
        return err
    }
    return nil
}
