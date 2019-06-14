build:
	unagi make orig@build
.PHONY: build

upload:
	unagi make orig@upload
.PHONY: upload

orig@build: ../build/phpmyadmin.tar.xz
	-rm -rf ../build/phpmyadmin
	cd ../build && unagi tar -Jxf phpmyadmin.tar.xz
	mv ../build/phpMyAdmin-4.6.3-all-languages ../build/phpmyadmin
	cp config/* ../build/phpmyadmin/
	sed -e "s/{{UNAGI_PASSWORD}}/$${UNAGI_PASSWORD}}/g" \
		../build/phpmyadmin/config.inc.php.template \
		> ../build/phpmyadmin/config.inc.php
.PHONY: build

orig@upload: orig@build
	cd ../build/phpmyadmin && \
		gcloud app deploy --project=icfpc-phpmyadmin --version=master --promote
.PHONY: upload

../build/phpmyadmin.tar.xz:
	unagi curl -o ../build/phpmyadmin.tar.xz \
		https://storage.googleapis.com/icfpc-public-package/phpmyadmin/phpMyAdmin-4.6.3-all-languages.tar.xz
