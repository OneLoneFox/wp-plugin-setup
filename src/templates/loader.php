<?php
/**
* Plugin Name: [[meta_plugin_name]]
* Description: [[meta_plugin_description]].
* Version: 1.0
* Author: LoneFox for Delindel
*/
defined( 'ABSPATH' ) || exit;

class [[plugin_class_name]]_Loader
{
    public static function init() 
    {
        require __DIR__ . '/vendor/autoload.php';
        [[plugin_class_name]]::instance();
    }
}
add_action( 'plugins_loaded', array('[[plugin_class_name]]_Loader', 'init'));

?>