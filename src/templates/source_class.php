<?php

defined( 'ABSPATH' ) || exit;

class [[plugin_class_name]]
{
    private static $instance = null;

    private function __construct(){}

    function init()
    {
        add_shortcode( '[[plugin_shortcode_tag]]', [$this, '[[plugin_shortcode_tag]]_shortcode'] );
    }

    function [[plugin_shortcode_tag]]_shortcode(){

    }

    public static function instance()
    {
        if(is_null(self::$instance)){
            self::$instance = new self;
            self::$instance->init();
        }
        return self::$instance;
    }
}