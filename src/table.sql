CREATE TABLE Camera_Calibrations (
  Id INTEGER PRIMARY KEY,
  camera TEXT NOT NULL,
  description TEXT NOT NULL,
  kmatrix integer[9]
);

-- couldn't use the following syntax??  
-- need more research on this
  kmatrix integer[3][3]


-- insert
INSERT INTO Camera_Calibrations VALUES (0001, 'IR', 'Boson');
INSERT INTO Camera_Calibrations VALUES (0002, 'Vis', 'Visible');
INSERT INTO Camera_Calibrations VALUES (0003, 'Vis', 'Nikon', '{1,2,3, 4,5,6,  7,8,9}' );

-- fetch 
SELECT * FROM Camera_Calibrations WHERE camera = 'Vis';

SELECT id, camera, description, kmatrix FROM Camera_Calibrations WHERE id = 3;

delete from Camera_calibrations where id = 5