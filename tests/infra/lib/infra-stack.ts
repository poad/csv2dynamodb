import { AttributeType, Table } from '@aws-cdk/aws-dynamodb';
import * as cdk from '@aws-cdk/core';
import { RemovalPolicy } from '@aws-cdk/core';

interface InfraStackProps extends cdk.StackProps {
  name: string
}

export class InfraStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props: InfraStackProps) {
    super(scope, id, props);

    new Table(this, 'CSV2DynamoDBTable', {
      tableName: props.name,
      partitionKey: {
        name: 'test',
        type: AttributeType.STRING
      },
      removalPolicy: RemovalPolicy.DESTROY
    });
  }
}
