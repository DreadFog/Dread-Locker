FROM php:8.0-apache
WORKDIR /var/www/html

COPY ./C2 ./
EXPOSE 80
RUN chmod 777 -R /var/www/html
RUN chown -R www-data:www-data /var/www/html
CMD ["apache2-foreground"]