#!/usr/bin/env node
import 'source-map-support/register';
import * as cdk from '@aws-cdk/core';
import { InfraStack } from '../lib/infra-stack';
import assert = require('assert');

const app = new cdk.App();

const name = app.node.tryGetContext('name') as string | undefined;
assert(name);

new InfraStack(app, `csv2dynamodb-test-${name}-stack`, {
  name
});
