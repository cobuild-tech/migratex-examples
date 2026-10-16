# PHP 7.3 with PDO MySQL, as in the XAMPP 7.3.19 the upstream dump was made with.
FROM php:7.3-apache
RUN docker-php-ext-install pdo_mysql
COPY php.ini /usr/local/etc/php/conf.d/xampp.ini
