using System.Collections;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEngine.UI;
//Esteban
public class StartGame : MonoBehaviour
{
	public Button start;

	// Use this for initialization
	void Start () {
	
		Button btn1 = start.GetComponent<Button>();
		btn1.onClick.AddListener(TaskOnClick);
	}

	
	void TaskOnClick()
	{
		SceneManager.LoadScene("Main");
	}

}
