.load ./dist/debug/geo0

select hex(geo_point(10, 20));
select geo_area('POLYGON((1 2,1 4,3 4,3 2,1 2))');
select geo_area('POLYGON((0.5 0.5,5 0,5 5,0 5,0.5 0.5), (1.5 1,4 3,4 1,1.5 1))');

select geo_within('POINT(0.5 0.5)', 'POLYGON((0 0, 0 1, 1 1, 1 0, 0 0))');
select geo_within(geo_point(.5, .5), 'POLYGON((0 0, 0 1, 1 1, 1 0, 0 0))');


.param set :mex_point '{"type":"Feature","geometry":{"type":"Point","coordinates":[-99.0736705,19.4337608]}}'
.param set :mex_rect '{"type":"Feature","geometry":{"type":"Polygon","coordinates":[[[-99.088859733959,19.446725078014737],[-99.05019339724888,19.446725078014737],[-99.05019339724888,19.423006334971177],[-99.088859733959,19.423006334971177],[-99.088859733959,19.446725078014737]]]}}'

select geo_within('POINT(-99.0736705 19.4337608)', :mex_rect);
select geo_within(geo_point(-99.0736705, 19.4337608), :mex_rect);
select geo_within(:mex_point, :mex_rect);

.header on
.mode box
select geo_as_geojson('POINT(-99.0736705 19.4337608)');
select geo_as_geojson('{"type":"Feature","geometry":{"type":"Point","coordinates":[-99.0736705,19.4337608]}}');
select geo_as_geojson('{"type":"Feature","properties":{"age": 100}, "geometry":{"type":"Point","coordinates":[-99.0736705,19.4337608]}}');