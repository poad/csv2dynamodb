import { AttributeType, Table } from '@aws-cdk/aws-dynamodb';
import * as cdk from '@aws-cdk/core';
import { RemovalPolicy } from '@aws-cdk/core';

export class InfraStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    new Table(this, 'CSV2DynamoDBTable', {
      tableName: `csv2dynamodb-test`,
      partitionKey: {
        name: 'test',
        type: AttributeType.STRING
      },
      timeToLiveAttribute: 'expire',
      removalPolicy: RemovalPolicy.DESTROY
    });
  }
}
