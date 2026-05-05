import * as Infra from '../lib/infra-stack';
import { Template } from 'aws-cdk-lib/assertions';
import * as cdk from 'aws-cdk-lib';
import { test } from 'vitest';

test('Empty Stack', () => {
  const app = new cdk.App();
  // WHEN
  const stack = new Infra.InfraStack(app, 'MyTest', {
    name: 'test',
  });
  // THEN
  const template = Template.fromStack(stack);
  template.templateMatches({
    'Resources': {},
  });
});

