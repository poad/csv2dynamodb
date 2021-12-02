import { AttributeType, Table } from 'aws-cdk-lib/aws-dynamodb';
import * as cdk from 'aws-cdk-lib';
import { RemovalPolicy } from 'aws-cdk-lib';
import { Construct } from 'constructs';

interface InfraStackProps extends cdk.StackProps {
  name: string
}

export class InfraStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props: InfraStackProps) {
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
