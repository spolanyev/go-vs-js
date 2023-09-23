//@author Stanislav Polaniev <spolanyev@gmail.com>

import { DynamoDBClient } from '@aws-sdk/client-dynamodb'
import { DynamoDBDocumentClient, GetCommand } from '@aws-sdk/lib-dynamodb'

async function getFileInfo(event) {
    console.log('Request:', event)

    const fileId = event.queryStringParameters?.fileId
    if (!fileId) {
        console.log('Error: no query parameter')
        throw new Error('no query parameter')
    }

    const documentClient = DynamoDBDocumentClient.from(new DynamoDBClient({}), {
        marshallOptions: {
            removeUndefinedValues: true,
        },
    })

    const getCommandInput = {
        TableName: 'fi-file-sharing',
        Key: {
            fileId: fileId,
        },
    }

    const command = new GetCommand(getCommandInput)
    const { Item } = await documentClient.send(command)

    if (!Item) {
        console.log('Error: item not found')
        throw new Error('item not found')
    }

    return {
        fileId: Item.fileId,
        name: Item.name,
        s3Key: Item.s3Key,
        preview: Item.preview,
        size: Item.size
    };
}

export const lambdaHandler = async (event) => {
    return await getFileInfo(event)
}
