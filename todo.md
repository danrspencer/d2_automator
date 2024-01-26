To set up a Webpack development server with HTTPS using a local certificate, you'll need to create or obtain a self-signed SSL certificate and then configure Webpack to use this certificate. Here's a step-by-step guide on how you can do this:

### 1. Generate a Self-Signed SSL Certificate

For development purposes, you can create a self-signed SSL certificate. You can use tools like OpenSSL to generate a certificate:

**Using OpenSSL:**

1. Open a terminal or command prompt.
2. Run the following command to generate a private key and a certificate:

   ```bash
   openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout localhost.key -out localhost.crt
   ```

3. Follow the prompts to complete the certificate creation. For "Common Name," use `localhost`.

This will create two files: `localhost.key` (the private key) and `localhost.crt` (the self-signed certificate).

### 2. Configure Webpack Development Server

After you have your SSL certificate and key, you need to configure the Webpack development server to use them.

**Modify Your Webpack Configuration:**

In your `webpack.config.js` or wherever you have your Webpack dev server configuration, add the following settings:

```javascript
const fs = require('fs');
const path = require('path');

// ... other configurations ...

devServer: {
    // ... other dev server settings ...
    https: {
        key: fs.readFileSync(path.resolve(__dirname, 'path/to/localhost.key')),
        cert: fs.readFileSync(path.resolve(__dirname, 'path/to/localhost.crt')),
    },
    // ... 
},
```

Replace `'path/to/localhost.key'` and `'path/to/localhost.crt'` with the actual paths to your generated key and certificate files.

### 3. Start Your Webpack Development Server

Run your development server as you usually do, for example using `npm start` or `yarn start`, depending on your setup.

### 4. Accessing Your Site

Open your web browser and go to `https://localhost:port`, where `port` is the port number your Webpack development server is running on. Since this is a self-signed certificate, your browser will likely warn you about the site's security certificate. You can proceed (usually by clicking "Advanced" and then "Proceed to localhost (unsafe)") as this is expected for self-signed certificates.

### Important Notes

- Self-signed certificates should only be used for development purposes. For production, always use a certificate issued by a trusted Certificate Authority (CA).
- Some browsers might require additional steps to accept self-signed certificates.
- Ensure your development environment is secure, especially when working with certificates and private keys.

This approach will allow you to run your Webpack development server over HTTPS using `https://localhost`.