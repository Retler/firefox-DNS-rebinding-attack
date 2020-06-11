package main

const home = `
<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <title>{{.Title}}</title>
  <script src="//code.jquery.com/jquery-1.11.1.min.js"></script>
  <link href="//netdna.bootstrapcdn.com/bootstrap/3.1.0/css/bootstrap.min.css" rel="stylesheet" id="bootstrap-css">
  <script src="//netdna.bootstrapcdn.com/bootstrap/3.1.0/js/bootstrap.min.js"></script>    
</head>
<body>

	<div class="container">
	  <div class="row">
			  <div class="col-md-6 text-center">
                <h1>{{.Header}}</h1>                
              </div>
      </div>
      {{range $i, $e := .Kittens}}
        {{if eq (mod $i 2) 0}}
		  <div class="row">
			  <div class="col-md-3">
                <img class="img-responsive" src="/images/{{inc $i}}.jpg" />
              </div>
			  <div class="col-md-3">
                <img class="img-responsive" src="/images/{{inc (inc $i)}}.jpg" />
              </div>
			</div>
        {{end}}
      {{end}}
		
	  <div class="row">
        <div class="col-md-6 text-center" id="ad-target" style="border-style: dotted;">
                          
        </div>
      </div>
	</div>
  
</body>
<script type="text/javascript" src="/js/someadscript.js" ></script>
</html>
`
