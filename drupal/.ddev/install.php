<?php

$installed = NULL;
try {
  $installed = \Drupal::state()->get('install_time');
}
catch (\Exception $e) {}
if (!$installed) {
  passthru('drush si -y');
  passthru('drush config-set "system.site" uuid "1c2345ae-2b2d-4400-b170-906ee1b43e48" -y');
  passthru('drupal entity:delete --all -n shortcut 1');
  passthru('drush cim -y');
}
