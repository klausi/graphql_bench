<?php

use Drupal\node\Entity\Node;

$node = Node::load(1);
if (!$node) {
  $node = Node::create([
    'type'  => 'article',
    'title' => 'Graphql Test',
  ]);
  $node->save();
}
