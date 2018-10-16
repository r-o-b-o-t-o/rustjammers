using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.SceneManagement;
using UnityEngine.UI;

public class StartGameButton : MonoBehaviour
{
	public Button m_YourFirstButton;
	// Use this for initialization
	void Start () {

		Button btn1 = m_YourFirstButton.GetComponent<Button>();
		btn1.onClick.AddListener(TaskOnClick);
	}
	
	// Update is called once per frame
	void Update () {
		
	}
	
	void TaskOnClick()
	{
		SceneManager.LoadScene("Main");
	}
	
}
