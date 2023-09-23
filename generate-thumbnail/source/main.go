//@author Stanislav Polaniev <spolanyev@gmail.com>

package main

import (
	"context"
	"errors"
	"fmt"
	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-sdk-go-v2/aws"
	"github.com/aws/aws-sdk-go-v2/config"
	"github.com/aws/aws-sdk-go-v2/feature/dynamodb/attributevalue"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb"
	"github.com/aws/aws-sdk-go-v2/service/dynamodb/types"
)

type FileInfo struct {
	FileId  string  `json:"fileId"`
	Name    string  `json:"name"`
	S3Key   string  `json:"s3Key"`
	Preview *string `json:"preview"`
	Size    *int    `json:"size"`
}

func GetFileInfo(ctx context.Context, request events.LambdaFunctionURLRequest) (FileInfo, error) {
	fmt.Printf("Request: %+v\n", request)

	failed := FileInfo{}

	fileId, ok := request.QueryStringParameters["fileId"]
	if !ok {
		fmt.Println("Error: no query parameter")
		return failed, errors.New("no query parameter")
	}

	defaultConfig, err := config.LoadDefaultConfig(ctx)
	if err != nil {
		fmt.Println("Error:", err)
		return failed, err
	}

	dbClient := dynamodb.NewFromConfig(defaultConfig)

	getItemInput := &dynamodb.GetItemInput{
		TableName: aws.String("fi-file-sharing"),
		Key: map[string]types.AttributeValue{
			"fileId": &types.AttributeValueMemberS{Value: fileId},
		},
	}

	getItemOutput, err := dbClient.GetItem(ctx, getItemInput)
	if err != nil {
		fmt.Println("Error:", err)
		return failed, err
	}

	if len(getItemOutput.Item) == 0 {
		fmt.Println("Error: item not found")
		return failed, errors.New("item not found")
	}

	result := FileInfo{}
	if err := attributevalue.UnmarshalMap(getItemOutput.Item, &result); err != nil {
		fmt.Println("Error:", err)
		return failed, err
	}

	return result, nil
}

func main() {
	lambda.Start(GetFileInfo)
}
