<?php

$installed = NULL;
try {
  $installed = \Drupal::state()->get('install_time');
}
catch (\Exception $e) {}
if (!$installed) {
  passthru('drush si -y');
}
