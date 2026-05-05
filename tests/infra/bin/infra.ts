#!/usr/bin/env node
import assert from 'assert';
import { InfraStack } from '../lib/infra-stack.js';
import * as cdk from 'aws-cdk-lib';

const app = new cdk.App();

const name = app.node.tryGetContext('name') as string | undefined;
assert(name);

new InfraStack(app, `csv2dynamodb-test-${name}-stack`, {
  name,
});
