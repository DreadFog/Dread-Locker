FROM php:8.0-apache
WORKDIR /var/www/html

COPY ./C2 ./
EXPOSE 80
CMD ["apache2-foreground"]