<?php

use Drupal\node\Entity\Node;

$node = Node::load(1);
if (!$node) {
  $node = Node::create([
    'type'  => 'article',
    'title' => 'Graphql Test',
    'uuid' => '6997b22e-36ef-4d6e-9683-af23f4e7f137',
  ]);
  $node->save();
}
