<?php

if (file_exists($app_root . '/' . $site_path . '/settings.ddev.php')) {
  include $app_root . '/' . $site_path . '/settings.ddev.php';
}

$config_directories[CONFIG_SYNC_DIRECTORY] = '../config/sync';
