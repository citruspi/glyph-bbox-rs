NODE_MODULES_PATH=./node_modules
WEB_VENDOR_PATH=./assets/web/vendor

deps:
	yarn install
	mkdir -p "${WEB_VENDOR_PATH}"
	cp "${NODE_MODULES_PATH}/raphael/raphael.min.js" "${WEB_VENDOR_PATH}"

clean:
	rm -rf "${NODE_MODULES_PATH}" "${WEB_VENDOR_PATH}"